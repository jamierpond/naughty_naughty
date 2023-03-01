# deploys this docker container to gcr.io and cloud run
# requires gcloud and docker to be installed

# if any command fails, exit
set -e

# if not sudo, exit
if [ "$EUID" -ne 0 ]
  then echo "Please run with sudo!"
  exit
fi

# build the docker image
docker build -t naughty . --platform linux/amd64

# tag the image
docker tag naughty gcr.io/naughty-379220/naughty

# push the image to gcr.io
docker push gcr.io/naughty-379220/naughty

# deploy the image to cloud run
# gcloud run deploy naughty --image gcr.io/naughty-379220/naughty --platform managed --region us-central1 --allow-unauthenticated
