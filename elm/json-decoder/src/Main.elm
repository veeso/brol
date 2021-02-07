module Main exposing (User)

import Browser
import Html exposing (..)
import Html.Attributes exposing (src)
import Http
import Json.Decode exposing (Decoder, bool, field, map4, maybe, string)


type alias User =
    { username : String
    , avatar : Maybe String
    , online : Bool
    , lastActvity : String
    }


main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }



-- Model
-- Our model is a state machine, with 3 states: Success when ready to show data, Loading when loading data, Error when unable to gather data


type Model
    = Success User
    | Loading
    | Error String



-- Update


type Msg
    = GotUser (Result Http.Error User)


update : Msg -> Model -> ( Model, Cmd Msg )
update msg _ =
    case msg of
        GotUser result ->
            case result of
                Ok user ->
                    -- Parse json
                    ( Success user, Cmd.none )

                Err err ->
                    ( Error (fmtHttpError err), Cmd.none )



-- Init


init : () -> ( Model, Cmd Msg )
init _ =
    ( Loading
    , Http.get
        { url = "http://localhost:3000/user"
        , expect = Http.expectJson GotUser userDecoder
        }
    )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions _ =
    Sub.none



-- View


view : Model -> Html Msg
view model =
    div []
        [ h2 [] [ text "User from remote" ]
        , viewUser model
        ]


viewUser : Model -> Html Msg
viewUser model =
    case model of
        Error err ->
            div []
                [ text ("Could not get users: " ++ err)
                ]

        Loading ->
            text "Loading..."

        Success user ->
            div []
                [ h3 [] [ text user.username ]
                , h4 [] [ text user.lastActvity ]
                , h4 []
                    [ text
                        (if user.online then
                            "Online"

                         else
                            "Offline"
                        )
                    ]
                , viewAvatar user.avatar
                ]


viewAvatar : Maybe String -> Html Msg
viewAvatar avatar =
    case avatar of
        Just imgSrc ->
            img [ src imgSrc ] []

        Nothing ->
            span [] [ text "This guy has no avatar" ]



-- User decoder


userDecoder : Decoder User
userDecoder =
    map4 User
        (field "username" string)
        (maybe (field "avatar" string))
        (field "online" bool)
        (field "lastActivity" string)


fmtHttpError : Http.Error -> String
fmtHttpError error =
    case error of
        Http.BadUrl url ->
            "The URL " ++ url ++ " was invalid"

        Http.Timeout ->
            "Unable to reach the server, try again"

        Http.NetworkError ->
            "Unable to reach the server, check your network connection"

        Http.BadStatus 500 ->
            "The server had a problem, try again later"

        Http.BadStatus 400 ->
            "Verify your information and try again"

        Http.BadStatus _ ->
            "Unknown error"

        Http.BadBody errorMessage ->
            errorMessage
