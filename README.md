# mini_rust_desk_relay_server 中继服务器

## 简介

mini_rust_desk_relay_server 是 客户端之间实现无缝远程连接的关键组件，尤其是在因 NAT 或防火墙限制无法直接进行点对点（P2P）连接时。它充当桥梁，确保通信可以在任何网络配置下进行。

## 特点

- **中继连接**：使网络连接能够绕过 NAT 和防火墙限制。
- **WebSocket 支持**：提供对 WebSocket 连接的支持，增强与 Web 客户端的兼容性。


### 命令行参数
- -p, --port=[NUMBER]：设置中继服务器的监听端口。如果未指定，将使用默认 TCP 21117 用于标准连接，TCP 21119 用于 WebSocket 连接
- -k, --key=[KEY]：限制只有提供匹配密钥的客户端才能访问。这通过防止未经授权的使用，增强了安全性


### 下面就是基本的交互日志
- 316.221.16.194是发起方
- 316.221.16.199是被控制方
- 后面的话就是数据单纯的转发

[2024-11-10 15:06:08.745215 +08:00] INFO [src\relay_server.rs:93]  relay request RequestRelay {
    id: "417866831",
    uuid: "429908f3-3e62-43d9-8d3a-ca3c3e9513b4",
    socket_addr: b"",
    relay_server: "",
    secure: false,
    licence_key: "mM+2AqccYg5imAbJaoKWcLAzcr6M4TG6g93y3xHani8=",
    conn_type: DEFAULT_CONN,
    token: "",
    special_fields: SpecialFields {
        unknown_fields: UnknownFields {
            fields: None,
        },
        cached_size: CachedSize {
            size: 0,
        },
    },
} from [::ffff:316.221.16.194]:54835
[2024-11-10 15:06:08.745280 +08:00] INFO [src\relay_server.rs:108] New relay request 429908f3-3e62-43d9-8d3a-ca3c3e9513b4 from [::ffff:316.221.16.194]:54835
[2024-11-10 15:06:08.806722 +08:00] INFO [src\relay_server.rs:93]  relay request RequestRelay {
    id: "",
    uuid: "429908f3-3e62-43d9-8d3a-ca3c3e9513b4",
    socket_addr: b"",
    relay_server: "",
    secure: false,
    licence_key: "OeVuKk5nlHiXp+APNn0Y3pC1Iwpwn44JGqrQCsWqmBw=",
    conn_type: DEFAULT_CONN,
    token: "",
    special_fields: SpecialFields {
        unknown_fields: UnknownFields {
            fields: None,
        },
        cached_size: CachedSize {
            size: 0,
        },
    },
} from [::ffff:316.221.16.199]:54690
[2024-11-10 15:06:08.806778 +08:00] INFO [src\relay_server.rs:100] Relayrequest 429908f3-3e62-43d9-8d3a-ca3c3e9513b4 from [::ffff:316.221.16.199]:54690 got paired
[2024-11-10 15:06:20.322177 +08:00] INFO [src\relay_server.rs:105] Relay of [::ffff:316.221.16.199]:54690 closed