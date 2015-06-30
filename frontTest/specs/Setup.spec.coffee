define (require) ->

    Setup = require '../../js/models/GameSetup'

    describe 'game', ->

        it 'should setup initial board', ->
            expectedInitialSetup = [
                [
                    {"rank":"1","side":3}
                    {"rank":"2","side":3}
                    {"rank":"3","side":3}
                    {"rank":"3","side":3}
                    {"rank":"4","side":3}
                    {"rank":"4","side":3}
                    {"rank":"4","side":3}
                    {"rank":"5","side":3}
                    {"rank":"5","side":3}
                    {"rank":"5","side":3}
                ]
                [
                    {"rank":"5","side":3}
                    {"rank":"6","side":3}
                    {"rank":"6","side":3}
                    {"rank":"6","side":3}
                    {"rank":"6","side":3}
                    {"rank":"7","side":3}
                    {"rank":"7","side":3}
                    {"rank":"7","side":3}
                    {"rank":"7","side":3}
                    {"rank":"8","side":3}
                ]
                [
                    {"rank":"8","side":3}
                    {"rank":"8","side":3}
                    {"rank":"8","side":3}
                    {"rank":"8","side":3}
                    {"rank":"9","side":3}
                    {"rank":"9","side":3}
                    {"rank":"9","side":3}
                    {"rank":"9","side":3}
                    {"rank":"9","side":3}
                    {"rank":"9","side":3}
                ]
                [
                    {"rank":"9","side":3}
                    {"rank":"9","side":3}
                    {"rank":"S","side":3}
                    {"rank":"B","side":3}
                    {"rank":"B","side":3}
                    {"rank":"B","side":3}
                    {"rank":"B","side":3}
                    {"rank":"B","side":3}
                    {"rank":"B","side":3}
                    {"rank":"F","side":3}
                ]
            ]

            setup = new Setup()

            expect(setup.get('board')).to.deep.equal expectedInitialSetup
