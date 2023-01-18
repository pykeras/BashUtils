# Some custom daily tools



### Random Password Generator

- [ ] **genpass** : _an easy to use free random password generator, never use `forget password` again_.

  - _Give it a name (server name), and a number (length for password) It will generate a new highly secure password and will save it on your system as an encrypted file so you will never forget your password again._

  - _On first use it will generate a key file for you which you must keep it safe and __please don't save it alongside your `genpass file`__, this key will be used for retrieving data from encrypted file or saving new password in it._

  - `CAUTION`: _As an example you can save `genpass` or encrypted file on your `phone/tablet/pc at home` and keep `1-time` generated key file on your `laptop`._ __Keep both safe or you will lose all your passwords__


  **USAGE :**

  * _first use :_

    ```bash
    $ python3 genpass init
    ----OR---
    # to specify encryption key path
    $ python genpass --init -f ./Documents/passgen -i ./Desktop/secret.key
    ```

    _The above command will generate an encrypted file in `Documents` with name of `passgen` and an encryption key in `Desktop` named `secret.key`._

    > `-f (optional):` _Path and filename for encrypted file (default current directory/folder)._ 
    >
    > `-i (optional):` _Path and filename to save key (default current directory/folder)._

  * _after that :_

      ```bash
      # generate new password (use defaults)
      $ python3 genpass.py -n "myEmail" 
      ---- OR ----
      # generate new password
      $ python3 genpass.py -i ./secret.key -f ./Document/passgen -n "myEmail" -l 20
      ---- OR ----
      # list all saved passwords
      $ python3 genpass.py -i ./secret.key --list
      ---- OR ----
      # show password for specific name
      $ python genpass.py -i ./secret.key --find "myEmail"
      ```
      
      > `-n:` _A name for new password for easier retrieve or remember_  
      >
      > `-i (optional):` _Path and filename to key, __auto created in first use__ (default current directory/folder)._
      >
      > `-f (optional):` _Path and filename for encrypted file (default current directory/folder)_  
      > `-l (optional):` _length of new random password $8$ or higher (default: $8$)_  
      > `-e (optional):` _if provided password may include `+=-_,.|\/{}()[]<>` characters._
      >
      > `-a (optional):` _list all names used for saved passwords._

_`Test environment`: `OS Linux`, `Python 3.8.10`_

----

