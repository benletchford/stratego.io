import sys
import os
from pynt import task


@task()
def test():
    os.system(
        'nosetests ./gae/tests ./gae --logging-level=ERROR --with-gae --gae-application="gae/app.yaml"')
