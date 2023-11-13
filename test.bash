PACKAGE_NAME="rxjs"
PACKAGE_VERSION="$(node -p -e "require('./package.json').version")"

FOUND_VERSION=$(npm info $PACKAGE_NAME versions)

if $FOUND_VERSION
then
    IS_NEW_VERSION=true
else
    IS_NEW_VERSION=false
fi

echo $IS_NEW_VERSION