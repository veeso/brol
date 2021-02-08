--  Elm-chat
--  Developed by Christian Visintin <christian.visintin1997@gmail.com>
--  Copyright (C) 2021 - Christian Visintin
--  Distribuited under "The Unlicense" license
--  for more information, please refer to <https://unlicense.org>


module Main exposing (..)

import Date
import Json.Decode exposing (Decoder, fail, succeed, string, field, maybe, andThen, bool, map4)


{-| Custom decoder for last activity parameter
-}
lastActivityDecoder : String -> Decoder Date.Date
lastActivityDecoder isodate =
    case Date.fromIsoString isodate of
        Ok dt ->
            succeed dt

        Err err ->
            fail ("Could not parse 'lastActivity': " ++ err)

{-
userDecoder : Decoder User
userDecoder =
    map4 User
        (field "username" string)
        (maybe (field "avatar" string))
        (field "lastActivity" string |> andThen lastActivityDecoder) -- NOTE: HERE HERE HERE
        (field "online" bool)
-}
