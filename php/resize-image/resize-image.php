<?php

/**
 * Developed by Christian Visintin
 * 
 * Resize image
 */

if (count($argv) < 5) {
  die("Usage <image> <output> <width> <height>");
}

$image = new Imagick($argv[1]);
$width = (int) $argv[3];
$height = (int) $argv[4];

$image->resizeImage($width, $height, 0, 0, true);
$image->writeImage($argv[2]);
 
?>
