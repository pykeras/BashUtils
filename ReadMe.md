# Some custom daily tools

<details>
<summary><h3>Random Password Generator</h3></summary>

- [x] **genpass**: _an easy-to-use free random password generator, never use `forget password` again_.
- _Give it a name (e.g. server name), It will generate a new highly secure password and will save it on your system as an encrypted file so you will never forget your password again._

- _On first use it will generate a key file for you which you must keep safe (**please don't save it alongside your `passgen.enc`**), this key will be used for retrieving data from the encrypted file or saving a new password in it._
  - `CAUTION`: _As an example, you can save `genpass` or the encrypted file on your `phone/tablet/pc/USB` and keep the `1-time` generated key file on your `laptop`._ **Keep both safe or you will lose all your passwords**

**USAGE:**

- _first time use:_

  ```bash
  $ python3 genpass.py --init
  ----OR----
  # to specify encryption key path
  $ python3 genpass --init -i ./Documents/passgen.enc -k ./Desktop/secret.key
  ----OR----
  # initialize and generate password all together
  $ python3 genpass --init -n MyEmail
  ```

  _The above command will generate an encrypted file in `Documents` with the name of `passgen` and an encryption key in `Desktop` named `secret.key`._

  > `-f (optional):` _Path and filename for encrypted file (default current directory/folder)._
  >
  > `-i (optional):` _Path and filename to save key (default current directory/folder)._
  >
  > `-n:` _A name for a new password for easier retrieval or remember (names cannot have space use` _`)\_

- _after that:_

  ```bash
  # generate new password
  $ python3 genpass.py -n myEmail
  ---- OR ----
  $ python3 genpass.py -k ./secret.key -i ./Document/passgen.enc -n "myEmail" -l 20 -e
  ---- OR ----
  # List all names
  $ python3 genpass.py -a
  ---- OR ----
  # find for specific name
  $ python3 genpass.py -k ./secret.key -f "myEmail"
  ---- OR ----
  # generate password without saving
  $ python3 genpass.py -l 20
  ```

  > `-n:` _A name for a new password for easier retrieval or remember (names cannot have space use` _`)\_
  >
  > `-k (optional):` _Path and filename to key, **auto-created in the first use** (default current directory/folder)._
  >
  > `-i (optional):` _Path and filename for encrypted file (default current directory/folder)_  
  > `-l (optional):` _length of a new random password $12$ or higher (default: $12$)_
  >
  > `-f (optional)`: _Find the password for the provided name_
  >
  > `-e (optional):` _if provided password may include `+=-_,.|\/{}()[]<>` characters.\_
  >
  > `-a (optional):` _list all names used for saved passwords._

_`Test environment`: `OS Linux`, `Python 3.8.10`_

**Recommendation:** \_make an alias in `~/.bashrc` for easier use.

</details>

<details>
<summary><h3>MusicGuitar Bestof collection crawler (Async Python)</h3></summary>

- [x] **musiccrawler**: _an easy-to-use music crawler written in Python with `asyncio`_

_Can be used as a tutorial for `asyncio, aiohttp, aiofiles`_

- _after crawling the project open `musiccrawler` directory and update `urls.txt` with the URL to the best collection of your favorite singer split by a new line._

- _only support URLs from [musicguitars.ir](https://musicguitars.ir/%da%af%d9%84%da%86%db%8c%d9%86-%d8%a2%d9%87%d9%86%da%af-%d9%87%d8%a7%db%8c-%d8%af%d8%a7%d8%b1%db%8c%d9%88%d8%b4-%d8%a7%d9%82%d8%a8%d8%a7%d9%84%db%8c-4/)_

- _to run the script you can use_

  ```bash
  $ python main.py 128
  ```

  > _$128$ is the music file quality (by default set to $320$)_

_`Test environment`: `OS Linux`, `Python 3.10.12`_

</details>
