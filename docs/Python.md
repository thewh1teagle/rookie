# Rookie Python Docs

## Install

```typescript
pip3 install -U rookiepy
```

## Basic Usage

```python
import rookiepy
cookies = rookiepy.chrome() # Load cookies from Chrome
```

## Logging

Logging level can be controlled by using the `logging` module

```python
import logging
logging.basicConfig()
logging.getLogger().setLevel(logging.DEBUG)
```

To fully disable `rookiepy` logging you can set the level to `CRITICAL`

```python
import logging
logging.getLogger().setLevel(logging.CRITICAL)
```
