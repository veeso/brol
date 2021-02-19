--  Elm utils
--  Developed by Christian Visintin <christian.visintin1997@gmail.com>
--  Copyright (C) 2021 - Christian Visintin
--  Distribuited under "DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE"


module FmtUtils exposing (..)

import DateFormat
import Http
import Time exposing (Posix, Zone)


{-| Format a date with syntax => 2021/02/13 15:49
-}
prettyDateFormatter : Zone -> Posix -> String
prettyDateFormatter =
    DateFormat.format
        [ DateFormat.yearNumberLastTwo
        , DateFormat.text "/"
        , DateFormat.monthFixed
        , DateFormat.text "/"
        , DateFormat.dayOfMonthFixed
        , DateFormat.text " "
        , DateFormat.hourFixed
        , DateFormat.text ":"
        , DateFormat.minuteFixed
        ]


{-| Format an HTTP error as a string
You can provide a function to call to handle BadStatus and return a different string for each status

    fmtHttpError Http.Timeout Nothing -> "Unable to reach the server, try again"
    fmtHttpError (Http.BadStatus 500) Nothing -> "The server had a problem, try again later"
    fmtHttpError (Http.BadStatus 500) Just mycb -> "My custom message for 500"

-}
fmtHttpError : Http.Error -> Maybe (Int -> String) -> String
fmtHttpError error handleStatus =
    case error of
        Http.BadUrl url ->
            "The URL " ++ url ++ " was invalid"

        Http.Timeout ->
            "Unable to reach the server, try again"

        Http.NetworkError ->
            "Unable to reach the server, check your network connection"

        Http.BadStatus httpcode ->
            case handleStatus of
                Just statusMsgRsolver ->
                    statusMsgRsolver httpcode

                Nothing ->
                    case httpcode of
                        500 ->
                            "The server had a problem, try again later"

                        400 ->
                            "Verify your information and try again"

                        401 ->
                            "Sorry, but you need to authenticate to do that"

                        403 ->
                            "Sorry, you're not allowed to do that"

                        _ ->
                            "There was an error in processing your request"

        Http.BadBody errorMessage ->
            errorMessage

