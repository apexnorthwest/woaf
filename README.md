# woaf
Offer and Accept files over HTTP/S anywhere you have a terminal!

> NOTE! This application is still in development, and many features mentioned are either broken or absent. This project is under active development, so come back soon and we'll have fully working builds for you :)

It's a common problem: you've got some file that you need to get from one machine to another and you don't have shared storage. You can't connect from one to another with sftp or smb because they're not homogenus. The person at the desk next to you needs to send you a file and nobody can find a flash drive.

Enter `woaf`, a next-generation, static, lightweight solution to this problem. With a single command you can offer a file or folder as a download over http/s, so any machine with a browser, wget, curl, or powershell cat download it. Conversely, you can use it in Accept mode. With just as much ease, your friend can upload a file via api call or via a web form.

Features include (these are all a work in progress):
- https support
- http/2 support
- upload and download limits (time, count, or size)
- on-disk or in-memory caching so you can choose higher speed or lower memory use
- Configurable U/I hints so you can comforably run `woaf` behind a proxy or load balancer

## Usage Instructions
Usage instuctions coming soon!

## Building it yourself
```sh
rustup update stable
rustup default stable
cargo install woaf
```

Info on how to build the code for the purpose of contributing to the project can be found in the [Contributor Guide](CONTRIBUTING.md)

## Bug Reports and Feature Requests
We welcome all feedback and bug reports. All such reports and requests should be created in the Issues on this repository. At all times you should follow the [code of conduct](CODE_OF_CONDUCT.md).

You should include the following information in all bug reports or support requests:
- Version of `woaf` you're using
- What OS and OS version you're running
- How you installed `woaf`
- Detailed steps of how to reproduce the issue. This includes the config file and cli options you use when producing the error. Please sanitize any sensitive or private information.

For all feature requests, please provide the version of the app you're currently using, what OS version you're running, and an example of what you'd like the feature to look like once implemented.

## Contributing
You may find our [code of conduct](CODE_OF_CONDUCT.md) and [contributor guidelines](CONTRIBUTING.md)
in this repo. We expect everyone to follow the guidelines and policies to ensure a positive,
productive, and enjoyable experience for all involved.

## License
Copyright 2026 Apex Northwest

```
Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

