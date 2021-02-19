--  Elm utils
--  Developed by Christian Visintin <christian.visintin1997@gmail.com>
--  Copyright (C) 2021 - Christian Visintin
--  Distribuited under "DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE"


module ValidationUtils exposing (..)

import Regex

{-| Check whether a string is alphanumerical

    isAlphanumerical "pippo97" -> True
    isAlphanumerical "mon-a" -> False

-}
isAlphanumerical : String -> Bool
isAlphanumerical check =
    Regex.contains (Maybe.withDefault Regex.never <| Regex.fromString "^[a-zA-Z0-9]+$") check


{-| Check whether a password is safe.
A password must contain:

    - at least one lower case character
    - at least one uppercase character
    - at least one special character
    - at least one number

and must be at least 8 characters length

    isPasswordSafe "foobar123" -> False
    isPasswordSafe "foobar" -> False
    isPasswordSafe "FoObAr0!97" -> True

-}
isPasswordSafe : String -> Bool
isPasswordSafe password =
    Regex.contains (Maybe.withDefault Regex.never <| Regex.fromString "^(?=.*[A-Z])(?=.*[!@#$&*])(?=.*[0-9])(?=.*[a-z]).{8,}$") password

