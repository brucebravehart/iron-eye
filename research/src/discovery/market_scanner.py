from dataclasses import dataclass
from typing import Iterable


@dataclass
class AssetCandidate:
    symbol: str
    score: float
    rationale: str


def rank_assets(candidates: Iterable[AssetCandidate]) -> list[AssetCandidate]:
    """Sort candidates by score descending for downstream review."""
    return sorted(candidates, key=lambda c: c.score, reverse=True)


def top_assets(candidates: Iterable[AssetCandidate], limit: int = 10) -> list[AssetCandidate]:
    """Return top N candidates after ranking."""
    return rank_assets(candidates)[:limit]
