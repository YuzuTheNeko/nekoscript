# Native Modules
Native modules are built on top of NekoScript, they're called during runtime using adapters and allow end user to interact with rust directly.

These allow us to not overload global functions and variables with unneeded / not used methods and other.

Each native method needs a definition that is used within NekoScript and the respective function that is called upon use.

