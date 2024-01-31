<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->

<a name="readme-top"></a>

<br />
<div align="center">
<h3 align="center">VIC API</h3>

  <p align="center">
    <br />
    <br />
    <a href="https://github.com/American-Expediting/vic-api/issues">Report Bug</a>
    ·
    <a href="https://github.com/American-Expediting/vic-api/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->

## About The Project

Rust Axum server built to expose vendor endpoints for use in AWS phone systems

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->

## Getting Started

### Prerequisites

You must have Rust installed

- Rust installation

  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

Follow the instructions below to get a local development server up and running.

1. Clone the repo
   ```sh
   git clone https://github.com/American-Expediting/vic-api.git
   ```
2. Run the project to install packages
   ```sh
   cargo run
   ```
3. Set an ACCESS_KEY environment variable
   ```
   export ACCESS_KEY="<your ACCESS_KEY>"
   ```
4. Set a DATABASE_URL environment variable
   ```
   export DATABASE_URL="<your DATABASE_URL>"
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->

## Contact

Jacob Bruce - j.bruce@amexpediting.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>
