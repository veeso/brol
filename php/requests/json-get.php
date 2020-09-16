<?php

/**
 * Developed by Christian Visintin
 * 
 * Send simple GET/POST requests
 */

if (count($argv) < 2) {
  die("Missing URL argument");
}

$curl = curl_init();
curl_setopt($curl, CURLOPT_URL, $argv[1]);
//Set CURLOPT_RETURNTRANSFER so that the content is returned as a variable.
curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
//Set CURLOPT_FOLLOWLOCATION to true to follow redirects.
curl_setopt($curl, CURLOPT_FOLLOWLOCATION, true);
// Request and connect timeout
curl_setopt($curl, CURLOPT_TIMEOUT, 30);
curl_setopt($curl, CURLOPT_CONNECTTIMEOUT, 30);

echo "Sending request to " . $argv[1] . PHP_EOL;
$data = curl_exec($curl);
$status = curl_getinfo($curl, CURLINFO_HTTP_CODE);
echo "Status: " . $status . PHP_EOL;
$data = json_decode($data, true);
if ($status === 200) {
  print_r($data);
}

curl_close($curl);
 
?>
