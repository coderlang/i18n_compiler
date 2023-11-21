# i18n_compiler

本项目用于在 Mac（x86,M1）, Windows 平台将 CSV 国际化文件，编译为 iOS 、 Android 、 Web 需要的国际化文件。

## 编译 bin 文件

执行 ```make build``` 将编译 Mac（x86,M1），Windows 3 个平台 i18n_compiler 命令，根据提示安装 rust 依赖。

## 集成 i18n_compiler 模块

```shell
git submodule add https://github.com/coderlang/i18n_compiler.git submodules/i18n_compiler
```

git config --global pull.rebase true
git config --global submodule.recurse true