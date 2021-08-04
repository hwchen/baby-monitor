# baby-monitor

rust on linux

- attaches to Usb mic using cpal audio lib
- monitors stream
- if baby is awake (according to heuristic), call phone
- phone number is determined by schedule, in config file

# setup
To find the device, I'm using
```
arecord -l
```
I'm sure there will be a better way in the future.
