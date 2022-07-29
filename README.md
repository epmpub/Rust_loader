安装部署：

  1 . 先安装Rust环境,用以下命令

  ```shell```
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  2 。 clone项目后，进入项目目录；

  3 .  用命令 cargo build --release 即可生成二进制文件，路进在 target\release下面；
  
  4. 将生成的二进制文件，copy到java包根目录即可；
