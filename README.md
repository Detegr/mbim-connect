# mbim-connect

Lately I've been using `mbimcli` instead of `NetworkManager` to connect to LTE network. `mbimcli` is a handy tool, but on thing it does not do automatically is assigning the IP configuration of the connection to a network device. This tool calls `mbimcli` to establish a new LTE connection, queries the IP configuration, and assigns it to a network device. However, it does *not* touch your `resolv.conf` because of multitude of ways of handling DNS server configurations. However, it does output the DNS servers returned by `mbimcli` for further processing.

## TODO:
In the current state, the connection is established with IPV4 only. Parsing the IPV6 configuration is implemented, but there is no way to choose which one to use.

## License
GPLv2
