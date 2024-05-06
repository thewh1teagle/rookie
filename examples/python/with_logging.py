import logging
import rookiepy

FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
logging.basicConfig(format=FORMAT)
logging.getLogger().setLevel(logging.DEBUG)
cookies = rookiepy.load()
print(f'found {len(cookies)} cookies')