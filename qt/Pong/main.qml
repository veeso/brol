import QtQuick 2.5
import QtQuick.Window 2.2

Window {
    id: root;
    visible: true;
    width: Screen.width;
    height: Screen.height;
    color: "#000000";

    //visibility: Window.FullScreen

    Rectangle {
        id: playField;
        width: parent.width - 256;
        x: 128;
        y: 64;
        height: parent.height - 128;
        color: "#101010";

        property int playerTwoX: (root.width - 256 - 48 - 32);
        property int playerOneX: (128 + 32);
        property int xBall: ball.y;

        Rectangle {
            id: playerOne;
            y: (parent.height / 2) - 64;
            x: 32;
            height: 128;
            width: 48;
            color: "#B7B7B7";

            /*NumberAnimation on y {
                id: movePlayerOne;
                from: playerOne.y;
                to: ball.y;
                loops: Animation.Infinite;
            }*/
        }

        Rectangle {
            id: playerTwo;
            y: (parent.height / 2) - 64;
            x: playField.width - 32 - 48;
            height: 128;
            width: 48;
            color: "#B7B7B7";

            NumberAnimation on y {
                id: movePlayerTwo;
                from: 0;
                to: playField.xBall;
                loops: Animation.Infinite;
            }
        }

        Image {
            id: ball;
            x: ((root.width - 256 - 48 - 32) / 2) - 32;
            y: (parent.height / 2) - 32;
            width: 64;
            height: 64;
            fillMode: Image.PreserveAspectCrop;
            source: "images/beach-ball.svg";

            NumberAnimation on x {
                id: goRight;
                from: ball.x;
                to: playField.playerTwoX;
                duration: 7000;
                onStopped: {
                    goLeft.start();
                }
            }
            NumberAnimation on y {
                id: goDown;
                from: ball.y;
                to: root.height - 256 - 24;
                duration: 5000;
                onStopped: {
                    goUp.start();
                }
            }
            NumberAnimation on x {
                id: goLeft;
                from: ball.x;
                to: playField.playerOneX - 64;
                duration: 7000;
                onStopped: {
                    console.log(playerOne.width + playerOne.x);
                    goRight.start();
                }
            }
            NumberAnimation on y {
                id: goUp;
                from: ball.y;
                to: 64;
                duration: 5000;
                onStopped: {
                    goDown.start();
                }
            }
        }
    }
}
