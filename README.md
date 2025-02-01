# big-brother
A nixpkgs tracker (soon-to also be a notifier) for checking merge statues/where a pull request is.
Currently barebones, but it gets the job done!

You can find the official instance at https://big-brother.sako.lol

## Usage
Run the executable and optionally change the host and port
```
big-brother --host 127.0.0.1 --port 1234
```

## Compiling
To compile install the rust toolchain and cargo and run
```
cargo build --release
```
Binary should be in the `target/release` folder.

## Installation
<details>
<summary>Nix</summary>
<br>
Add the flake as an input 

```nix
inputs = {
  big-brother.url = "github:snugnug/big-brother";
};
```

and then use it in your configuration.nix as so

```nix
  imports = [inputs.big-brother.nixosModules.default];
  
  services.big-brother = {
	  enable = true;
      port = 43523;
      environmentFile = "/srv/secrets/big-brother.env";
    };
  };
```

</details>

