## ninc

[English](../README.md) | 中文

翱翔门户命令行工具。

### 登录

```bash
ninc login -u [学号/手机号/邮箱/身份证号] -p [密码] [--save]
```

指定可选选项 `--save` 后将保存用户名和密码至配置文件 `config.toml` 中。

当配置文件中存在用户名信息时可以不用指定 `-u` 参数，密码同理。

登录后凭证将保存至 `storage.json` 文件中。

### 疫情填报

```bash
ninc esrep [-y] [-s (如果在学校)]
ninc esrap [-y] [-C [地址代号，即身份证前 6 位]] [-a [详细地址]] [-c [学院名称，如`信息类`]] [-p [电话号码]]
```

填报内容从配置文件中读取，若配置文件中不存在 `esrep` 则生成默认值，请务必对照[文档](./report_form.md)确保各项信息准确。

若当日已经填报过，将询问是否重复填报。指定可选选项 `-y` 后将跳过询问直接重复填报。

### 校园卡

```bash
ninc ecard
```

展示余额、月消费和最近消费记录。

```bash
ninc ecard -d [--begin [日期]] [--end [日期]] [--limit [最大展示数量]]
```

展示指定日期范围内的消费记录。

日期格式为 `YYYY-MM-DD`。若不指定 `--limit` 则默认为 10，不指定 `--end` 默认为今天，不指定 `--begin` 默认为 `--end` 的 30 天前。

### 帮助

```bash
ninc -h
ninc --help
ninc help
```

你可以通过以上三种方式获得帮助信息。

例如 `ninc login -h` 和 `ninc help login` 都可以获得登录相关的帮助信息。
