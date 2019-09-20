import Browser
import Html exposing (Html, Attribute, div, input, text, button)
import Html.Attributes exposing (..)
import Html.Events exposing (onInput)
import Regex



-- MAIN


main =
  Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
  { content : String
  }


init : Model
init =
  { content = "" }



-- UPDATE


type Msg
  = Change String


update : Msg -> Model -> Model
update msg model =
  case msg of
    Change newContent ->
      { model | content = newContent }

oodle : String -> String
oodle input = 
  case Regex.fromString "[aeiou]" of
      Nothing -> input
      Just matched -> Regex.replace matched (\_ -> "oodle") input 




-- VIEW


view : Model -> Html Msg
view model =
  div []
    [ input [ class "form-control", placeholder "Text to oodle", value model.content, onInput Change ] []
    , div [] [ text (oodle model.content) ]
    , button [ class "btn btn-primary"] [ text "button!" ]
    ]
