# Pandas Sports API Data Format

## Conventions

- Attributes preceded by an asterisk (`*`) are required.

### Matches

```json
{
    "start_time": datetime
    "duration_seconds": int,
    "location": string,
    *"team_1_id": int,
    *"team_2_id": int,
    *"team_1_score": int,
    *"team_2_score": int,
    *"details": [see "Match Details"]
}
```

### Match Details

Match details objects are polymorphic on their `sport` attribute, tagged internally as follows.

#### Baseball
```json
{
    *"sport": "baseball",
    "inning_runs": array of 9 or more int,
    "hits": int,
    "errors": int,
}
```

#### Basketball
```json
{
    *"sport": "basketball",
    "q1": int,
    "q2": int,
    "q3": int,
    "q4": int,
    "fgm": int,
    "fga": int,
    "tpm": int,
    "tpa": int,
    "ftm": int,
    "fta": int,
}
```

#### Football
```json
{
    *"sport": "football",
    "q1": int,
    "q2": int,
    "q3": int,
    "q4": int,
    "td": int,
    "fg": int,
    "p_att": int,
    "p_comp": int,
    "yds_pass": int,
    "yds_rush": int,
}
```

#### Hockey
```json
{
    *"sport": "hockey",
    sog: int,
    sm: int,
    fw: int,
    fl: int,
    sv: int,
    sa: int,
}
```

#### Soccer
```json
{
    *"sport": "soccer",
    "shots": int,
    "sog": int,
    "poss": float [0, 1]
    "passes": int,
    "fouls": int,
    "yellows": int,
    "red": int,
    "offsides": int,
    "corners": int,
}
```

#### Volleyball
```json
{
    *"sport": "volleyball",
    "set_scores": array of 3 to 5 int,
    "sv_ace": int,
    "sv_err": int,
    "sv_att": int,
    "at_kill": int,
    "at_err": int,
    "at_att": int,
    "rc_err": int,
    "rc_att": int,
}
```

### Teams
```json
{
    "team_name": string,
}```
