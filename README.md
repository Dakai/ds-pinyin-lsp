# 超，超简单的拼音输入法

![](https://user-images.githubusercontent.com/5492542/208365522-d69074ae-1328-42e6-9af1-0baee01e6e33.png)

中文 [English](./README-En.md)

## 介绍

通过 LSP 实现的超简单拼音输入法，其主要的用途是在 (neo)vim 编辑器中不需要切换输入法也能输入中文。
避免忘记切换输入法而导致在 Normal 模式下弹出输入法的蛋疼问题。

**注意**

- 非专业输入法，不是输入法的代替品，只适合少量需要输入中文的场景。
- 支持**全拼**和**双拼**， 需要配合 LSP 客户端使用，比如 coc.nvim (neo)vim / VS Code / lsp-bridge (Emacs) 等。

## Emacs 用户可以配合 [lsp-bridge](https://github.com/manateelazycat/lsp-bridge) 使用

`lsp-bridge` 已经添加了对 `ds-pinyin` 的支持

## (neo)vim 用户可以配合 coc.nvim 或者 neovim 内置的 lsp 使用

##### 1. 使用扩展 [coc-ds-pinyin-lsp](./packages/coc-ds-pinyin)

```
:CocInstall coc-ds-pinyin-lsp
```

插件设置项：

- `ds-pinyin-lsp.enabled`: 是否启用插件
- `ds-pinyin-lsp.trace.server`: 打开日志项
- `ds-pinyin-lsp.prompt`: 是否运行弹窗询问？比如询问下载 `ds-pinyin-lsp` `dict.db3` 文件
- `ds-pinyin-lsp.show_status_bar`: 是否开启状态栏显示
- `ds-pinyin-lsp.status_bar_flag`: 状态栏标志，默认 `Pinyin`
- `ds-pinyin-lsp.check_on_startup`: 是否检查更新
- `ds-pinyin-lsp.db_path`: `dict.db3` 文件
- `ds-pinyin-lsp.server_path`: `ds-pinyin-lsp` 命令或路经
- `ds-pinyin-lsp.completion_on`: 是否自动启用补全
- `ds-pinyin-lsp.completion_around_mode`: 是否启用环绕（光标在汉字（包括中文标点符号）开头/中间/结尾）补全模式
- `ds-pinyin-lsp.completion_trigger_characters`: 触发补全字符，配合 `completion_around_mode` 使用，在启用环绕模式后，可以通过输入触发补全字符启用自动补全
- `ds-pinyin-lsp.show_symbols`: 是否补全中文标点符号
- `ds-pinyin-lsp.show_symbols_only_follow_by_hanzi`: 是否只在中文后面补全中文符号
- `ds-pinyin-lsp.show_symbols_by_n_times`: 是否在输入 `n` 次符号后才显示中文符号补全选项，`0` 表示不开启先选
- `ds-pinyin-lsp.match_as_same_as_input`: 是否只显示完全匹配结果，比如: 输入 `pinyin` 会只显示 `拼音` 选项，不会显示 `拼音输入法` 等选项
- `ds-pinyin-lsp.match_long_input`: 是否显示长句匹配，比如：输入 `nihaonishishei` 在没有补全项的时候会把 `你好` `你是谁` 两个选项拼起来作为补全选项
- `ds-pinyin-lsp.shuangpin_scheme`: 双拼方案，目前支持 `off` (关闭) 和 `flypy` (小鹤双拼)
- `ds-pinyin-lsp.max_suggest`: 中文补全列表最大显示个数

插件命令：

- `ds-pinyin-lsp.turn-on-completion`: 开启自动补全
- `ds-pinyin-lsp.turn-off-completion`: 关闭自动补全
- `ds-pinyin-lsp.toggle-completion`: 切换自动补全

##### 2. 使用 neovim 内置的 lsp

如果你使用的是 [LazyVim](https://www.lazyvim.org/)，可以添加如下配置到 `lua/plugins/lsp.lua`:

```lua
return {
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        ds_pinyin_lsp = {
          init_options = {
            db_path = "your_path_to_database",
            completion_on = true,
            shuangpin_scheme = "flypy", -- 如果使用双拼
          },
        },
      },
    },
  },
}
```

否则，请先安装 [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig) 插件，具体配置可参考其[文档对应部分](https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#ds_pinyin_lsp)的配置。

如果需要配置选项，可参考如下的代码片段：

```lua
require('lspconfig').ds_pinyin_lsp.setup {
    init_options = {
        db_path = "your_path_to_database",
        completion_on = true,
        match_as_same_as_input = true,
        shuangpin_scheme = "flypy", -- 如果使用双拼，目前支持 "off" 和 "flypy"
    }
}
```

可配置项请参考 `coc-settings.json` 的 `initializationOptions` 部分。

##### 3. 不使用扩展

从 [Release](https://github.com/iamcco/ds-pinyin-lsp/releases/tag/v0.1.0) 下载 `ds-pinyin-lsp` 或
通过 `cargo install ds-pinyin-lsp` 安装 `ds-pinyin-lsp` 然后添加以下配置到 `coc-settings.json`

``` jsonc
  "languageserver": {
    "ds-pinyin": {
      "command": "path to ds-pinyin-lsp command",
      "filetypes": ["*"],
      "initializationOptions": {
        "db_path": "path to dict.db3",                             // dict.db3 字典文件
        "completion_on": true,                                     // 是否开启自动补全
        "shuangpin_scheme": "off",                                 // 双拼方案: off, flypy
        "completion_around_mode": false,                           // 是否启用环绕（光标在汉字（包括中文标点符号）开头/中间/结尾）补全模式
        "completion_trigger_characters": "",                       // 触发补全字符，配合 completion_around_mode 使用，在启用环绕模式后，可以通过输入触发补全字符启用自动补全
        "show_symbols": true,                                      // 是否补全中文标点符号
        "show_symbols_only_follow_by_hanzi": false,                // 是否只在中文后面补全中文符号
        "show_symbols_by_n_times": 0,                              // 是否在输入 `n` 次符号后才显示中文符号补全选项，`0` 表示不开启先选
        "match_as_same_as_input": true,                            // 是否只显示完全匹配结果，比如: 输入 `pinyin` 会只显示 `拼音` 选项，不会显示 `拼音输入法` 选项
        "match_long_input": true,                                  // 是否显示长句匹配，比如：输入 `nihaonishishei` 在没有补全项的时候会把 `你好` `你是谁` 两个选项拼起来作为补全选项
        "max_suggest": 50                                          // 中文补全列表最大显示个数
      }
    }
  }
```

> `dict.db3` 可以从 [Release](https://github.com/iamcco/ds-pinyin-lsp/releases/tag/v0.4.0) 下载。

可以通过向服务端发送通知（Notification）来关闭/开启/切换自动补全

- `$/turn/completion`: 参数: `{ completion_on?: boolean }`


## Packages

- [dict-builder](./packages/dict-builder) 用来构建 `dict.db3`
- [ds-pinyin-lsp](./packages/ds-pinyin-lsp) lsp 实现
- [coc-ds-pinyin-lsp](./packages/coc-ds-pinyin) coc.nvim 扩展

## 关于使用的字典

所使用的字典来自 [rime-ice](https://github.com/iDvel/rime-ice) 项目

### 请我吃个煎饼馃子 🤟

![btc](https://img.shields.io/keybase/btc/iamcco.svg?style=popout-square)

![image](https://user-images.githubusercontent.com/5492542/42771079-962216b0-8958-11e8-81c0-520363ce1059.png)
