#!/usr/bin/python3

from sys import argv, exit
import speech_recognition as sr

if __name__ == "__main__":
    if len(argv) < 2:
        print("Usage: <input_audio_file> [language]")
        exit(255)
    input_file = argv[1]
    language = 'it-IT'
    if len(argv) > 2:
        language = argv[2]
    speech_recognizer = sr.Recognizer()
    audio = sr.AudioFile(input_file)
    with audio as source:
        audio_file = speech_recognizer.record(source)
        result: str = speech_recognizer.recognize_google(audio_file, language = language)
        print("Recognized speech: '%s'" % result)
