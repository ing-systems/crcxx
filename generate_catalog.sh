#!/bin/sh

# Extracted from https://github.com/akhilles/crc-catalog/blob/master/generate_catalog.sh

echo "use super::Params;" >> src/crc8/catalog.rs
echo "use super::Params;" >> src/crc16/catalog.rs
echo "use super::Params;" >> src/crc32/catalog.rs
echo "use super::Params;" >> src/crc64/catalog.rs
echo "use super::Params;" >> src/crc128/catalog.rs

curl -s https://reveng.sourceforge.io/crc-catalogue/all.htm | grep -o 'width.*name.*"' | while read -r line; do
  # echo $(echo $line | \
  #   sed 's/ /, /g' | \
  #   sed 's/[-\/]/_/g' | \
  #   sed 's/width=\([0-9]*\), \(.*\), name="\(.*\)"/pub const \3: Algorithm<u\1> = Algorithm { \2 };/')

  width=$(echo $line | sed 's/width=\([0-9]*\) \(.*\) name="\(.*\)"/\1/')
  params=$(echo $line | sed 's/width=\([0-9]*\) \(.*\) name="\(.*\)"/\2/' | sed 's/ /, /g' | sed 's/=/: /g')
  name=$(echo $line | sed 's/width=\([0-9]*\) \(.*\) name="\(.*\)"/\3/' | sed 's/[-\/]/_/g')

  if [ $width -le 8 ]; then
    echo "pub const $name: Params<u8> = Params { width: $width, $params };" >> src/crc8/catalog.rs
  elif [ $width -le 16 ]; then
    echo "pub const $name: Params<u16> = Params { width: $width, $params };" >> src/crc16/catalog.rs
  elif [ $width -le 32 ]; then
    echo "pub const $name: Params<u32> = Params { width: $width, $params };"  >> src/crc32/catalog.rs
  elif [ $width -le 64 ]; then
    echo "pub const $name: Params<u64> = Params { width: $width, $params };"  >> src/crc64/catalog.rs
  elif [ $width -le 128 ]; then
    echo "pub const $name: Params<u128> = Params { width: $width, $params };"  >> src/crc128/catalog.rs
  fi
done
