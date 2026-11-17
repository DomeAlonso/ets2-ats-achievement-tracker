# ETS2 / ATS Achievement tracker 

> [!NOTE]
> This project is **Work In Progress (WIP)**.  
> Some achievements could be tracked incorrectly. Feel free to open an issue if you spot any mistakes. 


This project provides an overview of the progress for ETS2 and ATS achievements based on a savegame.
This is specially helpful on achievements that require you complete a number of specific jobs. E.g. _"Complete a delivery of at least 8 unique cargoes from the Heavy Cargo Pack"_. While steam tells you how many cargos you delivered it will not tell you which cargos are missing to complete the achievement.   

This project is a fork from [fangyi-zhou/sii-decode-rs](https://github.com/fangyi-zhou/sii-decode-rs) who provided the library to decode the savegames and other SII files as well as a baseline to track achievements in ETS2.  



## Usage

- Go to [https://domealonso.github.io/ets2-ats-achievement-tracker](https://domealonso.github.io/ets2-ats-achievement-tracker/#tracker)
- Upload your game.sii file.



### How to locate game.sii file in a ETS2 savegames.
- Go to ETS2 savegame location:
  - If Steam Cloud is not enabled for profile: Under windows this is `Documents/Euro Truck Simulator 2/profiles`
  - If Steam Cloud is not enabled for profile: Under windows this is `C:\Program Files (x86)\Steam\userdata\<STEAM-ID>\227300\remote\profiles\`
- *Select the ETS2 profile folder you want to retrieve*: The profile name is HEX encoded e.g. `4D792050726F66696C65`. If you don't know which folder belongs to the profile you are looking for, use [this tool](https://www.rapidtables.com/convert/number/hex-to-ascii.html) to lookup the profile name
- The game.sii file is stored under `save/autosave/game.sii`
