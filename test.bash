# PACKAGE_NAME="rxjs"
PACKAGE_NAME="@alshdavid/worker-shared-memory"
PACKAGE_VERSION="$(node -p -e "require('./package.json').version")"

FOUND_VERSION=$(npm info $PACKAGE_NAME version)
echo ok $?

if [ "$?" = "1"];then
    echo publish
fi

echo $FOUND_VERSION

if [ "$FOUND_VERSION" = "" ]; then
    IS_NEW_VERSION=true
else
    IS_NEW_VERSION=false
fi

echo $IS_NEW_VERSION