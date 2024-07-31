import androidx.compose.runtime.Composable
import coil3.ImageLoader
import coil3.annotation.ExperimentalCoilApi
import coil3.compose.setSingletonImageLoaderFactory
import coil3.request.crossfade
import coil3.util.DebugLogger
import di.koinModule
import org.koin.compose.KoinApplication
import org.koin.core.logger.KOIN_TAG
import org.koin.core.logger.Level
import org.koin.core.logger.Logger
import org.koin.core.logger.MESSAGE
import ui.nav.Navigation
import ui.theme.AppTheme

class PrintLogger(level: Level = Level.INFO) : Logger(level) {
    override fun display(level: Level, msg: MESSAGE) {
        println("[$level] $KOIN_TAG $msg")
    }
}

@OptIn(ExperimentalCoilApi::class)
@Composable
fun App() {
    setSingletonImageLoaderFactory { context ->
        ImageLoader.Builder(context)
            .crossfade(true)
            .logger(DebugLogger())
            .build()
    }

    KoinApplication(application = {
        logger(PrintLogger(level = Level.DEBUG))
        modules(koinModule)
    }) {
        AppTheme {
            Navigation()
        }
    }
}
