import sys
import os
from pynt import task


@task()
def test():
    os.system(
        'nosetests ./backTest ./app --logging-level=ERROR --with-gae --gae-application="app/app.yaml"')
