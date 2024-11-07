Projeto de Gerenciamento de Diretórios e Grupos

Este projeto é uma aplicação em Rust para gerenciar diretórios, arquivos, usuários e grupos de forma modular e organizada. Ele utiliza conceitos de programação orientada a objetos e boas práticas de modularização para facilitar a manutenção e a expansão futura do código.

Por que Modularizar?
A modularização permite que o código seja dividido em partes independentes e especializadas, chamadas "módulos". Cada módulo neste projeto cumpre uma função específica e trata de uma parte do sistema, como manipulação de usuários, grupos, arquivos e diretórios. Ao segmentar o código dessa forma, conseguimos:

   ->Facilidade de manutenção: Alterações e melhorias podem ser feitas em um módulo específico sem afetar outros.
   ->Reutilização de código: Funções e estruturas definidas em um módulo podem ser reutilizadas em diversas partes do projeto.
   ->Legibilidade: Com módulos bem definidos, o código torna-se mais legível e fácil de entender, especialmente para novos desenvolvedores.

Estrutura do Projeto
Abaixo está a estrutura dos arquivos e diretórios principais do projeto:
src/

    1.main.rs: Ponto de entrada do programa, contendo o menu principal e as chamadas para os outros módulos.
    2.funcoes.rs: Define as principais estruturas de dados e funções para manipulação de entidades como Usuario, Grupo, Diretorio, e Arquivo.
    3.menu_grupo.rs: Implementa o menu e as funções de manipulação dos grupos, como criação de grupo e adição/remoção de membros.
    4.menu_usuario.rs: Define o menu de operações relacionadas aos usuários, como criação de usuários e listagem de grupos do usuário.
    5.menu_diretorio.rs: Contém o menu e as funções de manipulação de diretórios e arquivos, incluindo operações de criação e gerenciamento.
    6.menu_arquivo: Define o menu de operações relacionadas aos arquivos.
    7.utils.rs: Contém funções utilitárias usadas em várias partes do projeto, como leitura de entrada do usuário.

Cada um desses módulos lida com uma parte específica do sistema, mantendo o código coeso e organizado.
Cargo.toml

O arquivo Cargo.toml é a configuração principal do projeto em Rust. Ele define as dependências, as configurações de compilação e as informações do pacote, como nome, versão e autor do projeto. O Cargo.toml permite ao Cargo, a ferramenta de gerenciamento de pacotes do Rust, compilar o projeto com as configurações adequadas e gerenciar bibliotecas externas.
Como Compilar e Executar

Para compilar e executar o projeto, é necessário ter o Rust e o Cargo instalados no sistema. Com eles instalados, siga os passos:

    1.Clone o repositório:
    git clone https://github.com/gabrielti/prog_imperativa_rust.git
    cd prog_imperativa_rust/tde2_rust/entrega

    2.Compile o projeto:
    cargo build

Isso gera um executável na pasta target/debug. Para gerar um binário otimizado para produção, use cargo build --release.

    3.Execute o projeto:
    cargo run

    4.Testes (opcional): Se o projeto tiver testes, você pode executá-los com:
    cargo test

Requisitos
Para compilar o projeto, é necessário:

    Rust (1.82 ou superior)
    Cargo (gerenciador de pacotes do Rust)
