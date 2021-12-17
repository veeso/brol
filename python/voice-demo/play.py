from pydub import AudioSegment
from pydub.playback import play
from sys import argv, exit

if __name__ == "__main__":
    if len(argv) < 2:
        print("Usage: <file>")
        exit(255)
    input_file = argv[1]
    # Open audio
    audio = AudioSegment.from_file(input_file)
    # Play
    try:
        play(audio)
    except KeyboardInterrupt:
        pass
