# Mark's Rust stuff

## The book
https://doc.rust-lang.org/stable/book/

## Installation
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

<details>
<summary>Dockerfile</summary>

```console
# Dockerfile with JetBrains CLion
FROM codercom/enterprise-base:ubuntu

# Run everything as root
USER root

# Packages required for multi-editor support
RUN apt-get update && \
    DEBIAN_FRONTEND="noninteractive" apt-get install -y \
    libxtst6 \
    libxrender1 \
    libfontconfig1 \
    libxi6 \
    libgtk-3-0

# Install JetBrains CLion.
RUN mkdir -p /opt/clion
RUN curl -L "https://download.jetbrains.com/product?code=CL&latest&distribution=linux" | tar -C /opt/clion --strip-components 1 -xzvf -

# Add a binary to the PATH that points to the clion startup script.
RUN ln -s /opt/clion/bin/clion.sh /usr/bin/clion

# configure script
COPY ["configure", "/coder/configure"]
RUN chmod +x /coder/configure

# Set back to coder user
USER coder

# Add Rust to path
ENV PATH /home/coder/.cargo/bin:${PATH}
```    
</details>

<details>
<summary>configure script (in Dockerfile)</summary>

```console
# add rust programming language
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# install VS Code extensions into code-server

SERVICE_URL=https://open-vsx.org/vscode/gallery ITEM_URL=https://open-vsx.org/vscode/item /var/tmp/coder/code-server/bin/code-server --install-extension rust-lang.rust
```
</details>

<details>
<summary>Update an installation</summary>

`rustup update`
</details>

## Config

<details>
<summary>CLion and IntelliJ Rust plugin</summary>

CLion/IntelliJ plug-ins are located in:
```console
~/.local/share/JetBrains/CLion2002.1/intellij-rust
JavaEWAH-1.1.13.jar                 markdown-jvm-0.3.1.jar
completion-ranking-rust-0.4.1.jar   org.eclipse.jgit-6.1.0.202203080745-r.jar
intelliLang.jar                     searchableOptions-0.4.172.4687-221.jar
intellij-rust-0.4.172.4687-221.jar  semver4j-3.1.0.jar
jackson-dataformat-toml-2.13.3.jar
```

https://plugins.jetbrains.com/plugin/8182-rust

</details>

<details>
<summary>VS Code extension</summary>

https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

</details>
