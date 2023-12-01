# SDCard 

The SDCard module has the responsibility of Log flight data to SD-Card card for recovery upon landing, 
as well as save Kalman Filter data for landing and for live recovery in case of reboot. Essentially it writes
and reads data, the challenge is the format, timing and stability of the system.

## State Driven

Currently, the SD-Card class is state-driven code, wherein each state is stored in an enum, and will be changed
based on specific conditions

Here we can see the different states of the class:
```cpp
  enum SDCARD_STATE {
    WAIT_SWITCH_PRESS = 0, // Waits for a switch to be pressed to activate SD_Card
    OPEN_FILE,
    UPLOAD_FROM_FILE
    WRITE_TO_FILE,
    CLOSE_FILE,
    IDLE,
    ERROR
  };
```

The SD-Card starts at the WAIT_SWITCH_PRESS state, where it waits for the switch to the SD-Card to be turned on

Upon being turned on, we will then open the file that the SD-Card reads/writes to

Then, we will upload the contents from the file to the Kalman Filter, for more info on this, see: <em>Recovery of Data</em>

Thereafter, we will want to be in a state of writing to the file, as we want to store the latest data for either recovery, or for statistics for the ground

There is another state for closing the file safely

And a state for idling, where we do not want to do anything

The last state is the ERROR state, wherein we set an error message and return <em>false</em> to Command Center



## Example State

Here we enter the WRITE_TO_FILE, where we call the FATFS_write method that is defined by harmony. If the write fails,
we enter setError, and give the Callee a chance to call getError() to print the error message we set.

If the write goes through, we check if we are done logging. If we are, we close the file. If not, we loop through the 
switch case again and re-enter writeToFile();
```cpp
int utils::SDCard::writeToFile() 
{
  if (!FATFS_write(sdCard.fileHandle, (const void*) sdCard.buffer, sdCard.nBytesToWrite, &sdCard.nBytesWritten))
  {
    setError("Failed to: WRITE TO FILE");
    return 0;
  }

    
  // Yet to define how I know if logging is complete
  if (logComplete())
    m_CurrentState = CLOSE_FILE;

  return 1;
}
```





## Recovery of data

The plan to recover data is to automatically recover it upon starting the proccess. Imagine we are turning it on for the first
time, right before a launch. In this case, the SD-Card will be empty, and no data will be loaded.

Afterwards, in the hypothetical world where the CPU reboots (which would never happen cause everything will go perfectly), and we need
to recover the data, the startup of the SD-Card automatically reboots the data from the SD-Card, which will now have relevant content to 
upload.

This method allows us several luxuries, as we can avoid several expensive calulations/checks on whether we need to reboot the data or not. It 
also saves us alot of development time as we lead up to the 2024 Launch.


