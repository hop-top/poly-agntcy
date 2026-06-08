<?php

declare(strict_types=1);

$opts = getopt('', ['endpoint:', 'target:']);
$endpoint = $opts['endpoint'] ?? 'http://localhost:8888';
$verb = $argv[1] ?? 'send';

if ($verb !== 'send') {
    fwrite(STDERR, "usage: php-agent.php send --target NAME [--endpoint URL]\n");
    exit(2);
}

$target = $opts['target'] ?? null;
if ($target === null) {
    fwrite(STDERR, "--target required\n");
    exit(2);
}

$payload = json_encode([
    'target'  => $target,
    'message' => 'hello from php',
], JSON_THROW_ON_ERROR);

$signature = base64_encode(hash('sha256', $payload, true));

echo "php-agent sent signed message to {$target} (endpoint={$endpoint}, sig={$signature})\n";
