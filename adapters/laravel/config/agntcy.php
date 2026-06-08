<?php

declare(strict_types=1);

return [
    'endpoint' => env('AGNTCY_DIR_ENDPOINT', 'https://directory.example'),
    'timeout' => (int) env('AGNTCY_TIMEOUT', 5),
];
