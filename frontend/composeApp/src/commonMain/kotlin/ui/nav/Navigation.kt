package ui.nav

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.automirrored.filled.ArrowBack
import androidx.compose.material3.DrawerValue
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.ModalNavigationDrawer
import androidx.compose.material3.rememberDrawerState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.rememberCoroutineScope
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import kotlinx.coroutines.launch
import ui.home.HomeScreen

@Composable
fun Navigation() {
    val navController = rememberNavController()
    val backIcon = @Composable {
        IconButton(
            onClick = { navController.navigateUp() }
        ) {
            Icon(
                imageVector = Icons.AutoMirrored.Filled.ArrowBack,
                contentDescription = null,
            )
        }
    }

    // TODO this causes a crash when resuming a process from disk, the stacktrace is:
    //
    // java.lang.IllegalStateException: Restoring the Navigation back stack failed: destination
    //         186859275 cannot be found from the current destination ComposeNavGraph(0x0)
    //         startDestination={Destination(0x19acea6) route=org.stypox.dicio.ui.nav.Home}
    //     at androidx.navigation.NavController.onGraphCreated(NavController.kt:1365)
    //
    // this is caused by a bug in the type-safe Compose Navigation library:
    // https://issuetracker.google.com/issues/341801005
    NavHost(navController = navController, startDestination = Home) {
        composable<Home> {
            ScreenWithDrawer(
                onSettingsClick = { /* TODO */ },
            ) {
                HomeScreen(it)
            }
        }
    }
}

@Composable
fun ScreenWithDrawer(
    onSettingsClick: () -> Unit,
    screen: @Composable (navigationIcon: @Composable () -> Unit) -> Unit
) {
    val drawerState = rememberDrawerState(initialValue = DrawerValue.Closed)
    val scope = rememberCoroutineScope()

    ModalNavigationDrawer(
        drawerState = drawerState,
        drawerContent = {
            DrawerContent(
                onSettingsClick = onSettingsClick,
                closeDrawer = {
                    scope.launch {
                        drawerState.close()
                    }
                }
            )
        },
    ) {
        screen {
            AppBarDrawerIcon(
                onDrawerClick = {
                    scope.launch {
                        drawerState.apply {
                            if (isClosed) open() else close()
                        }
                    }
                },
                isClosed = drawerState.isClosed,
            )
        }
    }
}
