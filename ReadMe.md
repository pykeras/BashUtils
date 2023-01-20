# Some custom daily tools



### Random Password Generator

- [x] **genpass**: _an easy-to-use free random password generator, never use `forget password` again_.
- _Give it a name (e.g. server name), It will generate a new highly secure password and will save it on your system as an encrypted file so you will never forget your password again._
  
- _On first use it will generate a key file for you which you must keep safe (__please don't save it alongside your `passgen.enc`__), this key will be used for retrieving data from the encrypted file or saving a new password in it._
    - `CAUTION`: _As an example, you can save `genpass` or the encrypted file on your `phone/tablet/pc/USB` and keep the `1-time` generated key file on your `laptop`._ __Keep both safe or you will lose all your passwords__



  **USAGE :**

  * _first time use:_

    ```bash
    $ python3 genpass.py --init
    ----OR----
    # to specify encryption key path
    $ python3 genpass --init -i ./Documents/passgen.enc -k ./Desktop/secret.key
    ----OR----
    # initialize and generate password all together
    $ python3 genpass --init -n MyEmail
    ```

    _The above command will generate an encrypted file in `Documents` with name of `passgen` and an encryption key in `Desktop` named `secret.key`._

    > `-f (optional):` _Path and filename for encrypted file (default current directory/folder)._ 
    >
    > `-i (optional):` _Path and filename to save key (default current directory/folder)._
    >
    > `-n:` _A name for new password for easier retrieve or remember (names cannot have space use` _`)_ 

  * _after that :_

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
      
      > `-n:` _A name for new password for easier retrieve or remember (names cannot have space use` _`)_ 
      >
      > `-k (optional):` _Path and filename to key, __auto created in first use__ (default current directory/folder)._
      >
      > `-i (optional):` _Path and filename for encrypted file (default current directory/folder)_  
      > `-l (optional):` _length of new random password $12$ or higher (default: $12$)_  
      >
      > `-f (optional)`: _Find password for provided name_
      >
      > `-e (optional):` _if provided password may include `+=-_,.|\/{}()[]<>` characters._
      >
      > `-a (optional):` _list all names used for saved passwords._

_`Test environment`: `OS Linux`, `Python 3.8.10`_

__Recommendation: __ _make an alias in `~/.bashrc` for easier use._

----

