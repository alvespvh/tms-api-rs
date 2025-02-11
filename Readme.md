Para otimizar o tempo de build em modo de desenvolvimento instalar as libs: clang-15 e mold

```
apt install clang-15 mold
```

Após instalar as libs adicionar a seguinte configuração no arquivo .cargo/config.toml:

```
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang-15"
rustflags = ["-C", "link-arg=--ld-path=/usr/bin/mold"]
```

Passos na criação do projeto:

```
1° - Criar o settings do projeto
```

```
2° - Criar a conexao com o banco de dados
```
