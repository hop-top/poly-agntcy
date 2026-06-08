<?php

declare(strict_types=1);

namespace HopTop\Agntcy\Dir\Spiffe;

/**
 * SPIFFE ID — URI of form spiffe://<trust-domain>/<path>.
 */
final class SpiffeId
{
    public function __construct(
        public readonly string $trustDomain,
        public readonly string $path,
    ) {
    }

    public static function parse(string $uri): self
    {
        if (!str_starts_with($uri, 'spiffe://')) {
            throw new \InvalidArgumentException('invalid SPIFFE id: '.$uri);
        }
        $rest = substr($uri, 9);
        $slash = strpos($rest, '/');
        if ($slash === false) {
            return new self($rest, '');
        }
        return new self(substr($rest, 0, $slash), substr($rest, $slash));
    }

    public function toString(): string
    {
        return 'spiffe://'.$this->trustDomain.$this->path;
    }
}
