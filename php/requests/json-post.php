<?php

/**
 * Developed by Christian Visintin
 * 
 * Send simple GET/POST requests
 */

if (count($argv) < 3) {
  die("Usage: ". $argv[0] . " <url> <body>");
}

$payload = $argv[2];
echo $payload . PHP_EOL;
$curl = curl_init();
curl_setopt($curl, CURLOPT_URL, $argv[1]);
curl_setopt($curl, CURLOPT_HTTPHEADER, array("Content-Type: application/json"));
curl_setopt($curl, CURLINFO_HEADER_OUT, true);
//Set CURLOPT_RETURNTRANSFER so that the content is returned as a variable.
curl_setopt($curl, CURLOPT_RETURNTRANSFER, true);
//Set CURLOPT_FOLLOWLOCATION to true to follow redirects.
curl_setopt($curl, CURLOPT_FOLLOWLOCATION, true);
// Request and connect timeout
curl_setopt($curl, CURLOPT_TIMEOUT, 30);
curl_setopt($curl, CURLOPT_CONNECTTIMEOUT, 30);
// Set body request
curl_setopt($curl, CURLOPT_POST, true);
curl_setopt($curl, CURLOPT_POSTFIELDS, $payload);
// Disable SSL check (FIXME: unsafe!)
curl_setopt($curl, CURLOPT_SSL_VERIFYHOST, false);
curl_setopt($curl, CURLOPT_SSL_VERIFYPEER, false);

echo "Sending request to " . $argv[1] . PHP_EOL;
$data = curl_exec($curl);
$status = curl_getinfo($curl, CURLINFO_HTTP_CODE);
echo "Status: " . $status . PHP_EOL;
if ($status === 200) {
  echo $data . PHP_EOL;
}

curl_close($curl);
 
?>
