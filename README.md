<div id="top"></div>

<!-- PROJECT SHIELDS -->
<div align="center">

[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]

[![codecov][codecov-shield]][codecov-url]

</div>

<!-- PROJECT LOGO -->
<br />
<div align="center">
<h3 align="center">Project Organizer</h3>

  <p align="center">
    Project Organizer is a CLI application for managing and creating projects. It allows the user to create project templates that are used to generate project files from.
    <br />
    <a href="https://github.com/frazzer951/ProjectOrganzier"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/frazzer951/ProjectOrganzier/issues">Report Bug</a>
    ·
    <a href="https://github.com/frazzer951/ProjectOrganzier/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>

- [About The Project](#about-the-project)
  - [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
    - [Option 1 - Easier Method](#option-1---easier-method)
    - [Option 2 - Manual Install](#option-2---manual-install)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

</details>

<!-- ABOUT THE PROJECT -->

## About The Project

ProjectOrganzier is a tool to help speedup the process or creating and organizing projects. By default, only an empty folder with the
project name will be created, but you can specify different options to populate the project folder. One option is to
clone and exiting git remote, or you can specify different templates to use.

<p align="right">(<a href="#top">back to top</a>)</p>

### Built With

- [Rust](https://www.rust-lang.org/)
- [Clap](https://github.com/clap-rs/clap)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

This is an example of how you may give instructions on setting up your project locally.
To get a local copy up and running follow these simple example steps.

### Prerequisites

- Optional - Cargo
  - https://rustup.rs/
  - This makes the installation process much easier, but is optional

### Installation

#### Option 1 - Easier Method

1. Install the cargo crate directly from GitHub
   ```sh
   cargo install --git https://github.com/Frazzer951/ProjectOrganzier.git
   ```
   To install a specific release version use `--tag <VERSION TAG>`

#### Option 2 - Manual Install

1. Download the binary from the latest release for your platform from
   the [release page](https://github.com/Frazzer951/ProjectOrganzier/releases)
2. Place the binary into a folder seen by your OSes PATH variable, so it can be called from anywhere

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- USAGE EXAMPLES -->

## Usage

Basic usage can be seen by looking at the help information for each command

```sh
project_organzier -h
```

To add a simple project use

```shell
project_organzier new -n MyProjectName
```

_For more examples, please refer to the [Documentation](https://github.com/Frazzer951/ProjectOrganzier/wiki)_

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any
contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also
simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes
   using [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) (`git commit -m 'feat: Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request to the develop branch

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- LICENSE -->

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Luke Eltiste - luke343279@gmail.com

Project Link: [https://github.com/frazzer951/ProjectOrganzier](https://github.com/frazzer951/fpm)

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/frazzer951/ProjectOrganzier.svg?style=for-the-badge
[contributors-url]: https://github.com/frazzer951/ProjectOrganzier/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/frazzer951/ProjectOrganzier.svg?style=for-the-badge
[forks-url]: https://github.com/frazzer951/ProjectOrganzier/network/members
[stars-shield]: https://img.shields.io/github/stars/frazzer951/ProjectOrganzier.svg?style=for-the-badge
[stars-url]: https://github.com/frazzer951/ProjectOrganzier/stargazers
[issues-shield]: https://img.shields.io/github/issues/frazzer951/ProjectOrganzier.svg?style=for-the-badge
[issues-url]: https://github.com/frazzer951/ProjectOrganzier/issues
[license-shield]: https://img.shields.io/github/license/frazzer951/ProjectOrganzier?style=for-the-badge
[license-url]: https://github.com/Frazzer951/ProjectOrganzier/blob/main/LICENSE
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?style=for-the-badge&logo=linkedin&colorB=555
[linkedin-url]: https://linkedin.com/in/luke-eltiste
[codecov-shield]: https://codecov.io/gh/Frazzer951/ProjectOrganzier/branch/main/graph/badge.svg?token=IFPJ06NXQ5
[codecov-url]: https://codecov.io/gh/Frazzer951/ProjectOrganzier
