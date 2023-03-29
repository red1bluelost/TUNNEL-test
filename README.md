```shell
sudo pppd -detach persist debug local noauth passive lock 192.168.10.1:192.168.10.100 /dev/ttyACM0 57600
```

```shell
sudo pppd -detach noipv6 debug noauth local passive persist mtu 200 lock 10.1.1.1:10.1.1.2 /dev/ttyACM0 57600
```

* -detach - no detach as background process
* debug - helpful print outs
* noauth - don't do any password/verifying
* local - do not use modem control line
* passive - keep trying to connect if not receiving valid LCP
* persist - keep trying to connect if the connection is terminated
* mtu 200 - limit the maximum transmission packet size
* lock - ensure exclusive access to serial device


pppd -detach local debug noauth passive lock persist 192.168.10.1:192.168.10.100 /dev/ttyS0 9600
pppd -detach local debug noauth passive lock 192.168.10.100:192.168.10.1 /dev/ttyUSB0 9600