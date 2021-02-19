--  Elm utils
--  Developed by Christian Visintin <christian.visintin1997@gmail.com>
--  Copyright (C) 2021 - Christian Visintin
--  Distribuited under "DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE"


module DecodeUtils exposing (..)

import File exposing (File)
import Json.Decode as Decode

{-| Get files from input at `on "change"` event
-}
getFilesFromInput : (List File -> msg) -> Decode.Decoder msg
getFilesFromInput message =
    Decode.map message <| Decode.at [ "target", "files" ] (Decode.list File.decoder)
