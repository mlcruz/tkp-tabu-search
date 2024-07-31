## Compilação

Para compilar/executar o programa, você precisa ter o cargo gerenciador de pacotes da linguagem rust instalado

1. https://rustup.rs/

   - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2. Dependendo da sua distribuição, as seguintes dependencias são necessarias:

- build-essentials -> apt install build-essentials
- cmake -> apt install cmake

## Comandos

cargo run --release -- $Path $Seed $Iterations $TabuListSize $NeighborHoodSize

exemplo:

cargo run --release -- tkp_instances/U2 12345 5000 10 10

o binario compilado gerado é salvo na pasta target/release/main

Para desabilitar o print de melhor solucão encontrada (para geração das tabelas dos relatorios), instancia a variavel de imbiente IGNORE_BEST com qualquer valor

export IGNORE_BEST=true
