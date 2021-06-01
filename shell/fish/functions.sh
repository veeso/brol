#!/usr/local/bin/fish

function fixdir;

  if [ -z "$argv[1]" ];
    echo "Usage: $_ <directory>"
    return 1
  end

  set DIR $argv[1]

  if [ ! -d "$DIR" ]
    echo "$DIR: no such file or directory"
    return 1
  end

  #Iterate over files
  for FILE in $DIR/*;
    #Check if directory; if directory fix child
    if [ -d $FILE ];
      chmod 755 $FILE
      set CHILD "$FILE"
      fixdir $CHILD
      continue
    end
    #Check if binary
    file $FILE | grep "interpreter" > /dev/null
    set IS_BINARY "$status"
    if [ "$IS_BINARY" -eq 0 ];
      #Set as executable
      chmod 755 $FILE
      continue
    end
    #Get file extension
    set EXT (string split -r -m1 . $FILE)[2]
    switch "$EXT"
      case "sh" "py" "exe"
        #Set as executable
        chmod 755 $FILE
      case '*'
        #Set as read-write
        chmod 644 $FILE
    end #End of switch
  end #End of loop
end #End of fixdir

function whatthecommit;
    curl --silent --fail http://whatthecommit.com/index.txt
end #End of whatthecommit

function pcommit;
    git log -1 --format=%h
end #pcommit

function pbranch;
    git symbolic-ref --short HEAD
end #pbranch

function docker-nginx;
    begin
        set DIR $argv[1]
        set PORT $argv[2]
        if [ -z $DIR ] || [ -z $PORT ]
            echo "Usage: docker-nginx <dir> <port>"
            return 255
        end
        set DIR (realpath $DIR)
        docker run --name nginx-$PORT -v $DIR:/usr/share/nginx/html:ro -p $PORT:80 -d nginx
    end
end # docker-nginx

function blank;
    set FILE $argv[1]
    if [ -z "$FILE" ]
        echo "Usage: blank <file>"
        return 255
    end
    echo "" > $FILE
end # blank

function url_encode;
    set INPUT $argv
    if [ -z "$INPUT" ]
        echo "Usage: url_encode <input>"
        return 255
    end
    echo $INPUT | jq -Rr @uri
end # url_encode

function gif;
    set TAG $argv
    if [ -z "$TAG" ]
        echo "Usage: gif <tag>"
        return 255
    end
    set TAG (url_encode $TAG)
    curl --silent "https://api.giphy.com/v1/gifs/random?tag=$TAG&api_key=0UTRbFtkMxAplrohufYco5IY74U8hOes" | jq .data.image_url | xargs -L 1 curl --silent | viu --height 32 -
end # gif
