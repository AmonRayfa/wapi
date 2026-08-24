# DNS Providers List

This file contains a list of DNS service providers supported by **Wapi**, along with their identifiers and support status. Identifiers are used to uniquely identify each provider in the client, while the support status indicates whether the provider is currently supported by the client or not.

Supported providers are marked with a white check mark (:white_check_mark:), while providers that are pending support are marked with an hourglass (:hourglass_flowing_sand:). Providers that cannot be supported due to restrictions or limitations (e.g., lack of public API) are marked with a red X (:x:). The list is updated regularly as new providers are added to the client.

| Provider Name                                          |   Identifier   |          Status          |
| :----------------------------------------------------- | :------------: | :----------------------: |
| [Alibaba Cloud](https://www.alibabacloud.com)          | `alibabacloud` | :hourglass_flowing_sand: |
| [bluehost](https://www.bluehost.com)                   |   `bluehost`   | :hourglass_flowing_sand: |
| [Cloudflare](https://www.cloudflare.com)               |  `cloudflare`  | :hourglass_flowing_sand: |
| [Crazy Domains](https://www.crazydomains.com)          |    **N.A.**    |           :x:            |
| [DNSPod](https://www.dnspod.com)                       |    `dnspod`    | :hourglass_flowing_sand: |
| [Domain.com](https://www.domain.com)                   |    **N.A.**    |           :x:            |
| [DreamHost](https://www.dreamhost.com)                 |  `dreamhost`   | :hourglass_flowing_sand: |
| [Dynadot](https://www.dynadot.com)                     |   `dynadot`    | :hourglass_flowing_sand: |
| [Enom](https://www.enom.com)                           |     `enom`     | :hourglass_flowing_sand: |
| [Epik](https://www.epik.com)                           |     `epik`     | :hourglass_flowing_sand: |
| [Gandi](https://www.gandi.net)                         |    `gandi`     | :hourglass_flowing_sand: |
| [GoDaddy](https://www.godaddy.com)                     |   `godaddy`    | :hourglass_flowing_sand: |
| [Hover](https://www.hover.com)                         |    `hover`     | :hourglass_flowing_sand: |
| [IONOS](https://www.ionos.com)                         |    `ionos`     | :hourglass_flowing_sand: |
| [Namecheap](https://www.namecheap.com)                 |  `namecheap`   | :hourglass_flowing_sand: |
| [NameSilo](https://www.namesilo.com)                   |   `namesilo`   | :hourglass_flowing_sand: |
| [OpenSRS](https://opensrs.com)                         |   `opensrs`    | :hourglass_flowing_sand: |
| [OVH](https://www.ovhcloud.com)                        |     `ovh`      | :hourglass_flowing_sand: |
| [Porkbun](https://porkbun.com)                         |   `porkbun`    |    :white_check_mark:    |
| [Register.com](https://www.register.com)               |    **N.A.**    |           :x:            |
| [ResellerClub](https://www.resellerclub.com)           | `resellerclub` | :hourglass_flowing_sand: |
| [Squarespace Domains](https://domains.squarespace.com) |    **N.A.**    |           :x:            |

You can also access the list of all the supported providers through the CLI by using the `wapi providers` command.

Any feedback or suggestions for new providers or existing ones are welcome.

## License

Copyright 2026 Amon Rayfa.

This project is licensed under the [Apache License (Version 2.0)](LICENSE).
