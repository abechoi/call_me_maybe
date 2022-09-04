<div align="center">
<h1>Call Me Maybe</h1>
<h2>SMS and Call Testing</h2>
<p>By Abe Choi</p>
</div>

<p>
A CLI Tool for connecting to Azure clusters via kubectl config, and quickly finding errors within pods, components, and helm releases.
</p>

1.  [Run App](#run-app)
2.  [Create Command](#create-command)
3.  [How to Use](#how-to-use)


## Run App

There are 2 binary executables available:

- MacOS Terminal `cmm`, enter `./cmm <subcommand> <from number> <to number>` to execute
- Windows Powershell `cmm.exe`, enter `.\cmm.exe <subcommand> <from number> <to number>` to execute.


### Create Command

To create a command for MacOS:

1. Copy cmm, `cmm`, into `/usr/local/bin`.
```
cp cmm /usr/local/bin
```

2. Check if path to `/usr/local/bin` exists.
```
echo $PATH
```

if `/usr/local/bin` string is not found in the output, create it.
```
export PATH=$PATH:/usr/local/bin
```

Repeat step 2. and verify `/usr/local/bin` is inside the PATH. If so, restart the terminal.

3. Call `cmm` command from directory.
```
cmm
```


### How to Use

Note: Add 2 more keys to keys.config, without these the app will not successfully call or text.

Note: Ensure tenant is SMS-enabled before using the text function.


To make a test call via command
```
cmm call <from number> <to number>
```

To make a test call via binary executable
```
./cmm call <from number> <to number>
```

To send a text via command
```
cmm text <from number> <to number>
```

To send a text via binary executable
```
./cmm text <from number> <to number>
```
