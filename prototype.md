GoXLR Utility 2.0 - Prototyping Doc

- **Device Manager**
    - Manages High Level Device information
    - Spawns a PnP handler for device attach / removal
    - Routes Commands / Messages to correct GoXLR
    - Messaging:
        - Incoming:
            - Receiver for PnP Device Attach / Remove (mpsc)
                - Sent from the spawned Platform PnP handler
            - Receiver for Device Handlers (mpsc)
                - 'Ready' once handler is ready to accept commands
                    - Serial Number - The serial number of the device
                    - A Messaging Channel to send commands
                - 'Error' state in the event something goes wrong
                    - Manager will remove state and attempt to 'Reset' the device, and spawn a new handler
                - 'Stopped' if the devices main loop has exited.
            - Receiver for Command Handling (mpsc)
                - These come from external entities
                    - External entities may include the web server, IPC client, global handler, etc.
        - Outgoing:
            - Message to device handle from external entity (oneshot)
                - Message comes in, message goes out, can't explain that
            - Message to device handle for information (oneshot)
                - For data such as DaemonStatus, expanding from 1.0 to allow for different types of replys!
            - Stop
                - Instructs the Device Handler to stop processing requests, and end its task
    - Data Managed:
        - USB Address -> Device Handle State (CREATED, READY, ERROR, STOPPED)
            - Primarily updated from the Device Handler, used to determine whether things like reinit are needed
        - Device Serial -> Messaging Channel
            - Updated from the Device Handler, used for routing messages into the task


- **PnP Handler**
    - Spawned by the Device Manager
    - May spawn other 'helper' tasks / threads depending on platform
    - Monitors and tracks devices then notifies of devices appearing and disappearing
    - Messaging:
        - Incoming:
            - 'Stop' signal (oneshot message, handled by spawner)
                - Called to inform the PnP handler it should stop processing
        - Outgoing:
            - Device Change Information (mpsc)
                - 'ATTACH (USB Address)' for a new device present
                - 'DETACH (USB Address)' for a device that's been removed
    - Data Managed:
        - Vec<USB Address>, list of connected devices for comparison
    - Platform Notes
        - rusb can be used on Linux, MacOS and Windows, but there may be better platform specific options
        - Windows: Optional HWND to trigger on WM_DEVICECHANGE instead of polling


- **Device Handler**
    - Spawned by the Device Manager
    - Spawns a USB Device Handler (TODO: Rename)
    - Represents the 'runtime' of a single GoXLR Device
    - Manages device profile, audio, all other parts of the device
    - Messaging:
        - Incoming:
            - Command (mpsc with oneshot responder)
                - Commands coming from external entities (such as the web server / ipc)
            - Device Event (mpsc)
                - ButtonDown(Button) - A button has been pressed
                - ButtonUp(Button) - A button has been released
                - VolumeChanged(Channel, volume) - A user has changed the volume of a channel
                - EncoderChanged(Encoder, value) - A user has changed the value of an encoder
            - Stop (oneshot)
                - Instructs the Handler to terminate and shutdown
        - Outgoing:
            - State
                - Sent to the Device Handler to indicate handler state (READY, ERROR, STOPPED)
            - GoXLR Command (mpsc with oneshot responder)
                - Commands sent out to the USB Device Handler (these control the GoXLR)
    - Data Managed:
        - Profile, audio, all aspects of a GoXLR device


- **USB Device Handler**
    - Spawned by the Device Handler
    - Spawns a button / event tracker
    - Handles All GoXLR Communication
    - Messaging:
        - Incoming:
            - Command (mpsc with oneshot responder)
                - Command to be executed against the GoXLR
            - Device Change Event (mpsc)
                - As with Device Handler, messages from the event tracker
            - Device Event (mpsc)
                - Received if something has critically failed
            - Stop (oneshot)
                - Instruction to shut down
        - Outgoing:
            - Device Event (mpsc)
                - As above, sending back to the Device Handler
            - State (mpsc)
                - Informing starter of run state
 