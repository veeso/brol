module Main exposing (User)

import Html exposing (..)
import Browser
import Http
import Json.Decode exposing (Decoder, bool, field, null, oneOf, string)
import Html.Attributes exposing (src)


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
                Ok json ->
                    -- Parse json
                    ( Success json, Cmd.none )

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
    [ h2 [] [ text "Random Cats" ]
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
        , h4 [] [ text ( if user.online then
              "Online"
            else
              "Offline"
           ) ] 
        , viewAvatar user.avatar
        ]

viewAvatar : Maybe String -> Html Msg
viewAvatar avatar = 
  case avatar of
    Just imgSrc ->
      img [src imgSrc] []
    Nothing ->
      img [src "/assets/fallback"] []

-- User decoder

userDecoder : Decoder User
userDecoder =
    field "username" string
    |> field "lastActivity" string
    |> field "online" bool
    |> field "avatar" (oneOf [ string, null ])

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
