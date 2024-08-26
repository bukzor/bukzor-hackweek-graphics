module Main exposing (main)

import Browser
import Html exposing (Html)
import Math.Vector2 exposing (Vec2, vec2)
import Math.Vector3 exposing (Vec3, vec3)
import WebGL


type alias Vertex =
    { position : Vec2
    , color : Vec3
    }


vertexShader : WebGL.Shader Vertex Model {}
vertexShader =
    [glsl|
        attribute vec2 position;
        attribute vec3 color;
        uniform vec3 vColor;
        void main () {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    |]


fragmentShader : WebGL.Shader {} Model {}
fragmentShader =
    [glsl|
        precision highp float;
        uniform vec3 vColor;
        void main () {
            gl_FragColor = vec4(vColor, 1.0);
        }
    |]


vertices : List Vertex
vertices =
    -- the vertices of the triangle
    [ { position = vec2 0 0.5, color = vec3 1 0 0 }
    , { position = vec2 0.5 -0.5, color = vec3 0 1 0 }
    , { position = vec2 -0.5 -0.5, color = vec3 0 0 1 }
    ]


buffer : WebGL.Mesh Vertex
buffer =
    -- convert vertices to WebGL buffers
    WebGL.lineLoop vertices


type alias Model =
    -- Define the model
    { vColor : Vec3 }


init : Model
init =
    -- the initial model
    { vColor = vec3 1 0 1 }


view : Model -> Html msg
view model =
    -- Define the view function
    WebGL.toHtml [] [ WebGL.entity vertexShader fragmentShader buffer model ]


main : Program () Model msg
main =
    -- the entry point
    Browser.sandbox { init = init, update = \_ model -> model, view = view }
